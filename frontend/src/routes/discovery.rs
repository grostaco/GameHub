use yew::{function_component, html, Html};

use crate::components::Nav;

#[function_component(Discovery)]
pub fn discovery() -> Html {
    html! {
        <>
        <Nav />
        <div style="padding-top: 2.5rem;" class="content">
        <h1>{"Recommended For You"}</h1>
        <div class="dflex" style="
            gap: 1rem;
            overflow: hidden;
        ">
            <div>
            <img src="./GameHub_files/soldierattack1300200.jpg" class="game-thumb" style="
            cursor: pointer;
        "/>
            <div class="game-title font-sm">{"Soldier Attack 1"}</div>
            <div class="dflex dflex-gap-tn">
                <span class="play-count"></span>
                <span class="play-count-text">{"69K"}</span>
            </div>
            </div>
            <div>
            <img src="./GameHub_files/soldierattack2300.jpg" class="game-thumb"/>
            <div class="game-title font-sm">{"Soldier Attack 2"}</div>
            <div class="dflex dflex-gap-tn">
                <span class="play-count"></span>
                <span class="play-count-text">{"69K"}</span>
            </div>
            </div>
            <div>
            <img src="./GameHub_files/soldierattack3-300.jpg" class="game-thumb"/>
            <div class="game-title font-sm">{"Soldier Attack 3"}</div>
            <div class="dflex dflex-gap-tn">
                <span class="play-count"></span>
                <span class="play-count-text">{"69K"}</span>
            </div>
            </div>
            <div>
            <img src="./GameHub_files/draculi300200.jpg" class="game-thumb"/>
            <div class="game-title font-sm">{"Draculi"}</div>
            <div class="dflex dflex-gap-tn">
                <span class="play-count"></span>
                <span class="play-count-text">{"69K"}</span>
            </div>
            </div>
            <div>
            <img src="./GameHub_files/spidersolitaire300200.jpg" class="game-thumb"/>
            <div class="game-title font-sm">{"Spider Solitaire"}</div>
            <div class="dflex dflex-gap-tn">
                <span class="play-count"></span>
                <span class="play-count-text">{"69K"}</span>
            </div>
            </div>
            <div>
            <img src="./GameHub_files/1010classic300200.jpg" class="game-thumb"/>
            <div class="game-title font-sm">{"1010 Classic HTML5"}</div>
            <div class="dflex dflex-gap-tn">
                <span class="play-count"></span>
                <span class="play-count-text">{"69K"}</span>
            </div>
            </div>
        </div>
        <h1>{"Popular"}</h1>
        <div class="dflex" style="
            gap: 1rem;
            overflow: hidden;
        ">
            <div>
            <img src="./GameHub_files/1400-pokemon-emerald-trashlock.jpg" class="game-thumb" style="
            cursor: pointer;
        "/>
            <div class="game-title font-sm">{"Soldier Attack 1"}</div>
            <div class="dflex dflex-gap-tn">
                <span class="play-count"></span>
                <span class="play-count-text">{"69K"}</span>
            </div>
            </div>
            <div>
            <img src="./GameHub_files/5444-squid-games-el-juego-del-calamar.jpg" class="game-thumb"/>
            <div class="game-title font-sm">{"Squid Game Simulator"}</div>
            <div class="dflex dflex-gap-tn">
                <span class="play-count"></span>
                <span class="play-count-text">{"69K"}</span>
            </div>
            </div>
            <div>
            <img src="./GameHub_files/ludo-hero_big.png" class="game-thumb"/>
            <div class="game-title font-sm">{"Ludo Hero"}</div>
            <div class="dflex dflex-gap-tn">
                <span class="play-count"></span>
                <span class="play-count-text">{"69K"}</span>
            </div>
            </div>
            <div>
            <img src="./GameHub_files/nice-to-z-u-300x200.jpg" class="game-thumb"/>
            <div class="game-title font-sm">{"Nice to Z You"}</div>
            <div class="dflex dflex-gap-tn">
                <span class="play-count"></span>
                <span class="play-count-text">{"69K"}</span>
            </div>
            </div>
            <div>
            <img src="./GameHub_files/95-1-300x200 (1).png" class="game-thumb"/>
            <div class="game-title font-sm">{"Axie Infinity"}</div>
            <div class="dflex dflex-gap-tn">
                <span class="play-count"></span>
                <span class="play-count-text">{"69K"}</span>
            </div>
            </div>
            <div>
            <img src="./GameHub_files/8438-pokemon-rubi.jpg" class="game-thumb"/>
            <div class="game-title font-sm">{"Pokémon Ruby and Sapphire"}</div>
            <div class="dflex dflex-gap-tn">
                <span class="play-count"></span>
                <span class="play-count-text">{"69K"}</span>
            </div>
            </div>
        </div>
        </div>
        </>
    }
}
