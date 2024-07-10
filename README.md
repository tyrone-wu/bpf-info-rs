# BPF metadata metrics

The original scope of this project was to create a "protocol agnostic" metrics library for BPF program metrics, intended for integrated into [bpftop](https://github.com/Netflix/bpftop) as a PR. The scope started to expand when I needed data on maps and links (attach point of the programs), so I decided to include metadata metrics on BPF objects from the `BPF_OBJ_GET_INFO_BY_FD` syscall command (which includes `bpf_prog_info`, `bpf_map_info`, `bpf_link_info`, and `bpf_btf_info`).

This project is divided into two parts:
- `bpf-info` library (inside `/bpf-metrics`), which collects and serializes metadata info on BPF programs, maps, and links into OpenMetrics and InfluxDB line protocol format.
- ~~Statsd exporter~~ Telegraf external plugin.

I originally planned to create an exporter for statsd with these metrics. However, it seems like statsd is starting to phase out. After some research, I'm opting for Telegraf as my metrics daemon/agent since I like the logo, and the InfluxDB line procotol solves some of the limitations I'm facing with the OpenMetrics exposition.

Due to some of the metric type contraints of the OpenMetrics spec, sending textual/string metrics is quite limiting. The `Info` type doesn't meet my usecase since the metric is static and is not part of a `MetricFamily`, and the `StateSet` is meant for enums. Ideally, if they added a metric type, called `Tag` or something, in which it's like `Info` but can be registered under a `MetricFamily`, then it would solve my usecase. Also, not much of a blocked, but I really like that the timestamp percision is down to the nanosecond.

To be honest, I'm not entirely sure if this will ever be complete. Maybe the line protol will, but the OpenMetrics format will likely be incomplete due to issue I described above.
