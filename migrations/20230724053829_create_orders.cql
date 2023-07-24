CREATE KEYSPACE IF NOT EXISTS orders
WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 };

USE orders;

CREATE TABLE IF NOT EXISTS bids (
                                    timestamp timestamp,
                                    exchange text,
                                    base text,
                                    quote text,
                                    price double,
                                    PRIMARY KEY ((base, quote), timestamp))
WITH CLUSTERING ORDER BY (timestamp DESC)
    AND COMPACTION = {'class': 'TimeWindowCompactionStrategy', 'base_time_seconds': 3600, 'max_sstable_age_days': 1};

CREATE TABLE IF NOT EXISTS asks (
                                    timestamp timestamp,
                                    exchange text,
                                    base text,
                                    quote text,
                                    price double,
                                    PRIMARY KEY ((base, quote), timestamp))
    WITH CLUSTERING ORDER BY (timestamp DESC)
     AND COMPACTION = {'class': 'TimeWindowCompactionStrategy', 'base_time_seconds': 3600, 'max_sstable_age_days': 1};