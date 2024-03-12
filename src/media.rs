pub struct Book {
    pub author_first_name: String,
    pub author_last_name: String,
    pub title: String,
    pub year_published: u32

}

impl Book {
	pub fn shout(&self) {
		println!("\nI am a drunken wombat!");
	}
}

impl Book {
	pub fn get_title(&self) -> String {
		String::from(&self.title)
	}
}

pub struct Movie {
	pub title : String,
	pub year_released: u32,
}

impl Movie {
	pub fn get_title(&self) -> String {
		String::from(&self.title)
	}
}



