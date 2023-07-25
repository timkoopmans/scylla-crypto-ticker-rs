CREATE KEYSPACE IF NOT EXISTS orders
WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 };

USE orders;

DROP TABLE IF EXISTS trades;
CREATE TABLE IF NOT EXISTS trades (
                                         exchange text,
                                         base text,
                                         quote text,
                                         time_bucket bigint,
                                         timestamp timestamp,
                                         id bigint,
                                         price double,
                                         qty double,
                                         PRIMARY KEY ((exchange, base, quote, time_bucket), timestamp, id)
) WITH CLUSTERING ORDER BY (timestamp ASC)
AND COMPACTION = {'class': 'TimeWindowCompactionStrategy', 'base_time_seconds': 3600, 'max_sstable_age_days': 1};

DROP TABLE IF EXISTS candles;
CREATE TABLE IF NOT EXISTS candles (
                                       exchange text,
                                       base text,
                                       quote text,
                                       time_bucket bigint,
                                       open_price double,
                                       high_price double,
                                       low_price double,
                                       close_price double,
                                       volume double,
                                       PRIMARY KEY ((exchange, base, quote), time_bucket)
) WITH CLUSTERING ORDER BY (time_bucket ASC);