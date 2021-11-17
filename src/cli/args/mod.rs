use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(short = "B", long = "book", default_value = "Genesis")]
    pub book: String,

    #[structopt(short = "C", long = "chapter", default_value = "1")]
    pub chapter: i32,

    #[structopt(short = "V", long = "verse", default_value = "1")]
    pub verse: i32,
}

impl Args {
    pub fn read() -> Args {
        Args::from_args()
    }
}
