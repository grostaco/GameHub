use yew::{function_component, html, Html};

#[function_component(Nav)]
pub fn nav() -> Html {
    html!(
        <div class="navbar">
            <img src="logo.png" height=47.64px />
            <a class="nav-title-element" href="#">{"Home"}</a>
            <a class="nav-title-element" href="#">{"Discover"}</a>
            <a class="nav-title-element" href="#">{"Friends"}</a>
            <a class="nav-title-element" href="#">{"Profile"}</a>
            <a class="nav-title-element" href="login.html">{"Login"}</a>
        </div>
    )
}
