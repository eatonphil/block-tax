# Block Tax

NOTE: This must be run on Linux. This will drop your page cache. Your
system will act as if there was a fork bomb but it's not a fork
bomb. It will get back to normal after you run your normal programs
once.

```
$ rustc blocktax.rs
$ ./blocktax | tee out.csv
$ duckdb -c "select column0, avg(column1) from 'out.csv' group by column0 order by avg(column1) asc"
┌──────────────────────────────┬────────────────────────┐
│           column0            │      avg(column1)      │
│           varchar            │         double         │
├──────────────────────────────┼────────────────────────┤
│ full_page_4096               │ 2.2344517999999948e-05 │
│ full_page_512                │ 2.3091620400000006e-05 │
│ partial_page_4096_6144_bytes │ 2.5642358400000015e-05 │
│ partial_page_512_20_bytes    │ 2.6037361999999975e-05 │
│ partial_page_512_768_bytes   │ 3.0276204400000106e-05 │
│ 2_full_page_4096             │ 3.2619195599999956e-05 │
│ partial_page_4096_20_bytes   │ 4.3916698800000013e-05 │
│ 2_full_page_512              │  4.912078600000008e-05 │
└──────────────────────────────┴────────────────────────┘
```
