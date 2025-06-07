use iced::application;
use yap::yap::Yap;

fn main() -> iced::Result {
    application(Yap::boot, Yap::update, Yap::view)
        .title(Yap::title)
        .theme(Yap::theme)
        .window(Yap::window_settings())
        .run()
}
