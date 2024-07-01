import {
    coinsEl, field, scoreEl, ship, startBtn, livesEl, shipContainer, shootRefillEl, hotkeysEl,
    gameOverEl, shopEl, shopItemContainer, shopItemTemplate, errorEl, loadingContainer, pauseIcon, playIcon
} from "./elements";
import Model, {
    Enemy, Projectile, GameState, Ship, Item
} from "./model";


export default class View {
    private model: Model;
    private lastShopItems: Item[] = [];
    constructor(model: Model) {
        this.model = model;
    }
    oldLives: number = 0;

    render() {
        if (this.model.isGameOver) {
            this.renderGameOverEl();
        } else {
            gameOverEl.classList.add("hidden");
        }

        this.renderHotkeys();

        this.displayIf(this.model.errorMessage !== undefined, errorEl);
        this.renderError();

        if (this.model.shopOpen) {
            shopEl.classList.add("open");
            this.renderShopItems();
        } else {
            shopEl.classList.remove("open");
        }

        this.renderShootRefillBar(this.model.shootRefillPercentage);

        field.style.width = this.model.field.width + "px";
        field.style.height = this.model.field.height + "px";

        if (this.oldLives !== this.model.lives) {
            this.renderHealth(this.model.lives, this.model.maxLives);
        }
        this.oldLives = this.model.lives;

        this.renderShip(this.model.ship, this.model.invulnerable);

        scoreEl.innerText = Math.round(this.model.score).toString();
        coinsEl.querySelector("span")!.innerText = this.model.coins.toString();

        this.renderStartBtn(this.model.gameState);

        this.removeElements(document.querySelectorAll(".enemy"));

        this.removeElements(document.querySelectorAll(".projectile"));

        this.model.enemies.forEach(this.renderEnemy.bind(this));

        this.model.projectiles.forEach(this.renderProjectile.bind(this));
    }


    private displayIf(condition: boolean, element: HTMLElement, className: string = "hidden") {
        if (condition) {
            element.classList.remove(className);
        } else {
            element.classList.add(className);
        }
    }

    private displayIfR(condition: boolean, element: HTMLElement, className: string = "open") {
        return this.displayIf(!condition, element, className);
    }

    private placeElement(element: HTMLElement, x: number, y: number, width: number, height: number) {
        element.style.left = x - width / 2 + "px";
        element.style.bottom = y - height / 2 + "px";
        element.style.width = width + "px";
        element.style.height = height + "px";
    }

    private querySelectorAll(element: HTMLElement, selector: string): HTMLElement[] {
        const elements = element.querySelectorAll(selector);
        return Array.from(elements) as HTMLElement[];
    }

    private renderError() {
        errorEl.innerText = this.model.errorMessage!;
    }

    private shopItemsAreEqual(items: Item[]) {
        if (this.lastShopItems.length !== items.length) {
            return false;
        }

        for (let i = 0; i < items.length; i++) {
            if (items[i]?.id !== this.lastShopItems[i]?.id) {
                return false;
            }
            if (items[i]?.level !== this.lastShopItems[i]?.level) {
                return false;
            }
        }
        return true;
    }

    private renderShopItems() {
        const items = this.model.shopItems;
        if (this.shopItemsAreEqual(items)) return;
        this.lastShopItems = items;

        shopItemContainer.innerHTML = "";
        items.forEach((item, index) => {

            const newItemEl = shopItemTemplate.cloneNode(true) as HTMLElement;

            newItemEl.id = `shop-item-${item.id}`;
            newItemEl.classList.remove("hidden");

            const tooltip = newItemEl.querySelector(".shop-item-tooltip") as HTMLElement;

            tooltip.style.zIndex = index + "";

            const itemInfo = newItemEl.querySelector(".shop-item-info") as HTMLElement;

            itemInfo.style.zIndex = index + 1 + "";

            for (const image of this.querySelectorAll(newItemEl, ".shop-item-image")) {
                if (image) {
                    (image as HTMLImageElement).src = `/assets/images/shop-items/${item.id}.svg`;
                }
            }

            for (const nameEl of this.querySelectorAll(newItemEl, ".item-name")) {
                if (nameEl) {
                    nameEl.innerText = item.title;
                }
            }


            const descriptionEl = newItemEl.querySelector(".description") as HTMLParagraphElement;
            descriptionEl.innerText = item.description;

            const priceEl = newItemEl.querySelector(".shop-item-price") as HTMLParagraphElement;
            const priceContainer = newItemEl.querySelector(".shop-item-price-container") as HTMLElement;
            if (item.level >= item.max_level) {
                priceEl.innerText = "Max";
                priceContainer.classList.add("max-level");
            } else {
                priceEl.innerText = item.price.toString();
            }

            const levelContainer = newItemEl.querySelector(".shop-item-level")! as HTMLElement;
            levelContainer.innerHTML = "";

            levelContainer.style.zIndex = index + 1 + "";

            for (let i = 0; i < item.max_level; i++) {
                const level = document.createElement("div");
                level.className = "level";
                if (i < item.level) {
                    level.classList.add("active");
                } else {
                    level.classList.add("inactive");
                }
                levelContainer.appendChild(level);
            }

            // TODO: Ist das an dieser Stelle richtig? Wie kriege ich es in den Controller?
            newItemEl.addEventListener("click", () => {
                this.model.buyItem(item.id);
            })

            shopItemContainer.appendChild(newItemEl);
        })
    }

    private renderShootRefillBar(percentage: number) {
        shootRefillEl.querySelector("div")!.style.width = percentage + "%";
    }

    private removeElements(elements: NodeListOf<HTMLElement>) {
        elements.forEach((element) => {
            element.remove();
        })
    }

    private renderShip(shipInfo: Ship, isInvulnerable: boolean) {
        shipContainer.style.left = shipInfo.x - shipInfo.width / 2 + "px";
        shipContainer.style.bottom = shipInfo.y - shipInfo.height / 2 + "px";
        ship.style.width = shipInfo.width + "px";
        ship.style.height = shipInfo.height + "px";
        ship.style.transform = `rotate(${shipInfo.get_angle() + 0.8}rad)`;

        if (isInvulnerable) {
            ship.classList.add("invulnerable");
        } else {
            ship.classList.remove("invulnerable");
        }
    }

    private renderStartBtn(gameState: GameState) {
        this.displayIf(gameState === GameState.Running, pauseIcon);
        this.displayIf(gameState !== GameState.Running, playIcon);
        const textEl = startBtn.querySelector("span")!;
        switch (gameState) {
            case GameState.Running:
                textEl.innerText = "Pause";
                break;
            case GameState.Paused:
                textEl.innerText = "Resume";
                break;
            case GameState.NotRunning:
                textEl.innerText = "Start new game";
                break;

            default:
                break;
        }
    }

    private renderProjectile(projectile: Projectile) {


        const projectileElement = document.createElement("div");
        projectileElement.id = `projectile-${projectile.id}`;
        projectileElement.className = "projectile";
        field.appendChild(projectileElement);


        const angle = Math.atan2(projectile.dy, projectile.dx)

        this.placeElement(projectileElement, projectile.x, projectile.y, projectile.radius, projectile.radius);
        projectileElement.style.transform = `rotate(${angle}rad)`;
    }

    private renderEnemy(enemy: Enemy) {

        const enemyElement = document.createElement("div");
        enemyElement.id = `enemy-${enemy.id}`;
        enemyElement.className = "enemy";
        enemyElement.classList.add(enemy.enemy_type.toString().toLowerCase());
        field.appendChild(enemyElement);

        this.placeElement(enemyElement, enemy.x, enemy.y, enemy.radius, enemy.radius);
    }

    // For each live render a heart. All lost lives are grey hearts, the rest are red.
    private renderHealth(lives: number, maxLives: number) {
        livesEl.innerHTML = "";

        for (let i = 0; i < maxLives; i++) {
            const heart = document.createElement("img");
            heart.className = "heart";

            if (i < lives) {
                heart.classList.add("red");
                heart.src = "/assets/images/heart_red.svg";
            } else {
                heart.classList.add("grey");
                heart.src = "/assets/images/heart_grey.svg";
            }

            livesEl.appendChild(heart);
        }
    }

    private renderHotkeys() {
        hotkeysEl.innerHTML = "";

        const hotkeys = this.model.hotkeys;

        hotkeys.forEach((hotkey) => {
            const hotkeyEl = document.createElement("div");
            hotkeyEl.className = "hotkey";
            hotkeyEl.innerHTML = `
                <div class="hotkey__key">${hotkey.key}</div>
                <div class="hotkey__description">${hotkey.description}</div>
            `;
            hotkeysEl.appendChild(hotkeyEl);
        });
    }

    private renderGameOverEl() {
        const coinEl: HTMLSpanElement = gameOverEl.querySelector(".coins")!;
        const scoreEl: HTMLSpanElement = gameOverEl.querySelector(".score")!;

        coinEl.innerText = this.model.coins.toString();
        scoreEl.innerText = Math.round(this.model.score).toString();

        gameOverEl.classList.remove("hidden");
    }
}