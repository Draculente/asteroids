import { field, startBtn, gameOverEl, shopCloseBtn, errorEl, loginScreen, loggedInEl, loginScreenBtn, loginScreenForm, loginBtn, registerBtn, logoutButton, deleteAccountBtn, endGameBtn } from "./elements";
import View from "./view";
import Model, { GameState, key } from "./model";

const model = new Model();
const view = new View(model);

const hotkeys = [
    {
        key: "S",
        description: "Toggle Shop",
        predicate: key("s"),
        callback: (event: any) => {
            event.preventDefault();
            model.toggleShop();
        }
    },
    {
        key: "Enter",
        description: "Start/Pause",
        predicate: key("Enter"),
        callback: (event: any) => {
            event.preventDefault();
            model.toggleGameState();
        }
    },
    {
        key: "Space",
        description: "Shoot",
        predicate: key(" "),
        callback: (event: any) => {
            event.preventDefault();
            model.shoot();
        }
    },
]

model.registerHotkeys(hotkeys);

const gameLoopIntervall = setInterval(() => {
    model.tick();
    view.render();
}, 20);

startBtn.addEventListener("click", () => {
    model.toggleGameState();
    view.render();
});

gameOverEl.addEventListener("click", () => {
    model.toggleGameState();
    view.render();
});

shopCloseBtn.addEventListener("click", () => {
    model.toggleShop();
    view.render();
});

window.addEventListener("mousemove", (e) => {
    const rect = field.getBoundingClientRect();
    const x = (e.clientX - rect.left) - model.ship.x;
    const y = (e.clientY - rect.top) - model.ship.y;

    model.rotateShip(x, y);
    view.render();
});

field.addEventListener("mousedown", (e) => {
    model.shoot();
}, true);

errorEl.addEventListener("click", () => {
    model.dismissError();
});


loginScreen.addEventListener("click", (e) => {
    if (e.target === loginScreen) {
        model.closeLoginScreen();
    }
})

loginScreenBtn.addEventListener("click", () => {
    if (!model.loggedIn) {
        model.openLoginScreen();
    }
})


window.addEventListener("keydown", (e) => {
    model.handleHotkey(e)
})

enum LoginScreenAction {
    Login,
    Register
}

function loginScreenSubmit(action: LoginScreenAction) {
    return async () => {
        const form = new FormData(loginScreenForm);
        const username = form.get("username") as string;
        const password = form.get("password") as string;
        if (action === LoginScreenAction.Login) {
            await model.login(username, password);
        } else {
            model.register(username, password);
        }
    }
}

loginBtn.addEventListener("click", loginScreenSubmit(LoginScreenAction.Login));

registerBtn.addEventListener("click", loginScreenSubmit(LoginScreenAction.Register));

loginScreenForm.addEventListener("submit", (e) => {
    e.preventDefault();
    loginScreenSubmit(LoginScreenAction.Login)();
})

logoutButton.addEventListener("click", () => {
    model.logout();
});

deleteAccountBtn.addEventListener("click", () => {
    model.deleteAccount();
})

endGameBtn.addEventListener("click", () => {
    model.endGame();
})

window.onbeforeunload = (e) => {
    clearInterval(gameLoopIntervall);
    model.saveGame();
    if (model.gameState !== GameState.NotRunning)
        e.preventDefault()
}


const width = window.innerWidth;
const height = window.innerHeight;
if ((width < 1350 && height < 920) || (height < 850) || (width < 1115)) {
    alert("The game is not optimized for your screen size. Please use a screen with a resolution of at least 1115x850 pixels.")
}