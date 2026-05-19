use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "gh-terminal", about = "国赫电台控制终端")]
pub struct Cli {
    /// 串口路径
    #[arg(short, long, default_value = "COM1")]
    pub port: String,

    /// 波特率
    #[arg(short = 'b', long, default_value_t = 115200)]
    pub baud: u32,

    /// HTTP 服务端口
    #[arg(short = 'p', long, default_value_t = 9100)]
    pub http_port: u16,

    /// 日志级别
    #[arg(long, default_value = "info")]
    pub log_filter: String,

    /// 状态轮询间隔 (ms)
    #[arg(long, default_value_t = 250)]
    pub poll_interval_ms: u64,
}
