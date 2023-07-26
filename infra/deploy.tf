provider "aws" {
  region = "us-west-2"
}

resource "aws_ecs_cluster" "cluster" {
  name = "scylla-cluster"
}

resource "aws_ecs_task_definition" "scylla" {
  family                = "scylla"
  network_mode          = "awsvpc"
  cpu                   = "256"
  memory                = "512"
  requires_compatibilities = ["FARGATE"]
  execution_role_arn    = aws_iam_role.execution_role.arn
  task_role_arn         = aws_iam_role.task_role.arn

  container_definitions = <<DEFINITION
  [
    {
      "name": "scylla",
      "image": "scylladb/scylla",
      "essential": true,
      "portMappings": [
        {
          "containerPort": 9042,
          "hostPort": 9042
        }
      ],
      "logConfiguration": {
        "logDriver": "awslogs",
        "options": {
          "awslogs-group" : "${aws_cloudwatch_log_group.scylla_log_group.name}",
          "awslogs-region": "us-west-2",
          "awslogs-stream-prefix": "ecs"
        }
      }
    }
  ]
  DEFINITION
}


resource "aws_service_discovery_private_dns_namespace" "private_namespace" {
  name = "scylla-namespace"
  vpc  = aws_vpc.my_vpc.id
}

resource "aws_service_discovery_service" "scylla_service" {
  name = "scylla-discovery-service"

  dns_config {
    namespace_id = aws_service_discovery_private_dns_namespace.private_namespace.id

    dns_records {
      ttl  = 10
      type = "A"
    }

    routing_policy = "MULTIVALUE"
  }

  health_check_custom_config {
    failure_threshold = 1
  }
}

resource "aws_ecs_service" "scylla" {
  name            = "scylla"
  cluster         = aws_ecs_cluster.cluster.id
  task_definition = aws_ecs_task_definition.scylla.arn
  desired_count   = 1
  launch_type     = "FARGATE"

  service_registries {
    registry_arn   = aws_service_discovery_service.scylla_service.arn
  }

  network_configuration {
    subnets          = [aws_subnet.subnet1.id, aws_subnet.subnet2.id]
    security_groups  = [aws_security_group.sg.id]
    assign_public_ip = true
  }
}

resource "aws_cloudwatch_log_group" "scylla_log_group" {
  name = "/ecs/scylla"
  retention_in_days = 14
}

resource "aws_cloudwatch_log_group" "ticker_log_group" {
  name = "/ecs/ticker"
  retention_in_days = 14
}


resource "aws_ecs_task_definition" "ticker" {
  family                = "ticker"
  network_mode          = "awsvpc"
  cpu                   = "256"
  memory                = "512"
  requires_compatibilities = ["FARGATE"]
  execution_role_arn    = aws_iam_role.execution_role.arn
  task_role_arn         = aws_iam_role.task_role.arn

  container_definitions = <<DEFINITION
  [
    {
      "name": "ticker",
      "image": "timkoopmans/scylla-crypto-ticker",
      "essential": true,
      "portMappings": [
        {
          "containerPort": 8000,
          "hostPort": 8000
        }
      ],
      "environment": [
        {
          "name": "DATABASE_URL",
          "value": "${aws_service_discovery_service.scylla_service.name}.scylla-namespace:9042"
        },
        {
          "name": "ROCKET_ADDRESS",
          "value": "0.0.0.0"
        },
        {
          "name": "FORCE_REDEPLOY",
          "value": "${timestamp()}"
        }
      ],
      "logConfiguration": {
        "logDriver": "awslogs",
        "options": {
          "awslogs-group" : "${aws_cloudwatch_log_group.ticker_log_group.name}",
          "awslogs-region": "us-west-2",
          "awslogs-stream-prefix": "ecs"
        }
      }
    }
  ]
  DEFINITION
}

resource "aws_lb" "alb" {
  name               = "ticker-alb"
  internal           = false
  load_balancer_type = "application"
  security_groups    = [aws_security_group.sg.id]
  subnets            = [aws_subnet.subnet1.id, aws_subnet.subnet2.id]
  enable_deletion_protection = false

  tags = {
    Name = "ticker-alb"
  }
}

resource "aws_lb_listener" "listener" {
  load_balancer_arn = aws_lb.alb.arn
  port              = "80"
  protocol          = "HTTP"

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.tg.arn
  }
}

resource "aws_lb_target_group" "tg" {
  name     = "ticker-tg"
  port     = 8000
  protocol = "HTTP"
  vpc_id   = aws_vpc.my_vpc.id
  target_type = "ip"

  health_check {
    enabled             = true
    interval            = 30
    path                = "/metrics"
    timeout             = 5
    healthy_threshold   = 3
    unhealthy_threshold = 2
    protocol            = "HTTP"
  }
}

resource "aws_ecs_service" "ticker" {
  name            = "ticker"
  cluster         = aws_ecs_cluster.cluster.id
  task_definition = aws_ecs_task_definition.ticker.arn
  desired_count   = 1
  launch_type     = "FARGATE"

  network_configuration {
    subnets          = [aws_subnet.subnet1.id, aws_subnet.subnet2.id]
    security_groups  = [aws_security_group.sg.id]
    assign_public_ip = true
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.tg.arn
    container_name   = "ticker"
    container_port   = 8000
  }

  depends_on = [aws_lb_listener.listener]
}

