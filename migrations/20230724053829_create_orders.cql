CREATE KEYSPACE IF NOT EXISTS orders
WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 };

USE orders;

DROP TABLE bids;
CREATE TABLE IF NOT EXISTS bids (
                                    timestamp timestamp,
                                    exchange text,
                                    base text,
                                    quote text,
                                    price double,
                                    amount double,
                                    PRIMARY KEY ((base, quote), timestamp))
WITH CLUSTERING ORDER BY (timestamp DESC)
AND COMPACTION = {'class': 'TimeWindowCompactionStrategy', 'base_time_seconds': 3600, 'max_sstable_age_days': 1};

DROP TABLE asks;
CREATE TABLE IF NOT EXISTS asks (
                                    timestamp timestamp,
                                    exchange text,
                                    base text,
                                    quote text,
                                    price double,
                                    amount double,
                                    PRIMARY KEY ((base, quote), timestamp))
WITH CLUSTERING ORDER BY (timestamp DESC)
AND COMPACTION = {'class': 'TimeWindowCompactionStrategy', 'base_time_seconds': 3600, 'max_sstable_age_days': 1};