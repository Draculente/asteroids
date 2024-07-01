import { GameRessource } from "../bindings/GameRessource";
import { ItemLevelRessource } from "../bindings/ItemLevelRessource";
import { Enemy, Field, GameModel, GameState, Item, Projectile, Ship } from "../pkg";
import HotkeyHandler, { Hotkey } from "./hotkeys";
import ViewModel from "./view_model";

export * from "./hotkeys";
export * from "../pkg";

export default class Model {
    private gameModel: GameModel;
    private viewModel: ViewModel;
    private hotkeyModel: HotkeyHandler;
    private lastFrameGameOver: boolean = false;
    private gameOverCallbacks: (() => void)[] = [];


    constructor() {
        this.gameModel = GameModel.new();
        this.viewModel = new ViewModel();
        this.hotkeyModel = new HotkeyHandler();
    }

    registerHotkeys(hotkeys: Hotkey[]) {
        this.hotkeyModel.registerHotkeys(hotkeys);
    }

    registerOnGameOver(callback: () => void) {
        this.gameOverCallbacks.push(callback);
    }

    handleHotkey(e: KeyboardEvent) {
        this.hotkeyModel.handleHotkey(e);
    }

    tick(): void {
        this.gameModel.tick();
    }

    async startGame() {
        this.gameModel.start_game();
    }

    toggleGameState(): void {
        if (this.gameState === GameState.NotRunning) {
            this.startGame();
        } else if (this.gameState === GameState.Paused) {
            this.resumeGame();
        } else {
            this.pauseGame();
        }
    }

    shoot(): void {
        this.gameModel.shoot();
    }

    rotateShip(dx: number, dy: number): void {
        this.gameModel.rotate_ship(dx, dy);
    }

    pauseGame(): void {
        this.gameModel.pause_game();
    }

    resumeGame(): void {
        this.gameModel.resume_game();
    }

    buyItem(itemId: number): void {
        this.gameModel.buy_item(itemId);
    }

    toggleShop(): void {
        this.viewModel.shopIsOpen = !this.viewModel.shopIsOpen;
        if (this.shopOpen) {
            this.pauseGame();
        } else if (this.gameState === GameState.Paused) {
            this.resumeGame();
        }
    }

    dismissError(): void {
        this.gameModel.dismiss_error();
    }

    async endGame() {
        this.gameModel.end_game();
    }

    // Getters

    get hotkeys(): Hotkey[] {
        return this.hotkeyModel.hotkeys;
    }

    get isGameOver(): boolean {
        let gameOver = this.gameState === GameState.NotRunning && this.lives <= 0;
        if (gameOver && !this.lastFrameGameOver) {
            this.gameOverCallbacks.forEach(callback => callback());
        }
        this.lastFrameGameOver = gameOver;
        return gameOver;
    }

    get errorMessage(): string | undefined {
        return this.gameModel.get_error_message();
    }

    get gameState(): GameState {
        return this.gameModel.get_game_state();
    }

    get score(): number {
        return this.gameModel.get_score();
    }

    get ship(): Ship {
        return this.gameModel.get_ship();
    }

    get enemies(): Enemy[] {
        return this.gameModel.get_enemies();
    }

    get projectiles(): Projectile[] {
        return this.gameModel.get_projectiles();
    }

    get field(): Field {
        return this.gameModel.get_field();
    }

    get coins(): number {
        return this.gameModel.get_coins();
    }

    get lives(): number {
        return this.gameModel.get_lives();
    }

    get maxLives(): number {
        return this.gameModel.get_max_lives();
    }

    get shootRefillPercentage(): number {
        return this.gameModel.get_shoot_refill_percentage();
    }

    get invulnerable(): boolean {
        return this.gameModel.is_invulnerable();
    }

    get shopItems(): Item[] {
        return this.gameModel.get_shop_items();
    }

    get shopOpen(): boolean {
        return this.viewModel.shopIsOpen;
    }

    get gameRessource(): GameRessource {
        const itemLevels: ItemLevelRessource[] = this.shopItems.map(item => {
            return {
                level: item.level,
                item: {
                    id: item.id,
                    name: item.title,
                    price: item.internal_price,
                    description: item.description,
                }
            }
        })
        return {
            id: this.gameModel.get_id() ?? null,
            enemy_spawn_timeout: this.gameModel.get_enemy_spawn_timeout(),
            ended: this.isGameOver,
            score: this.score,
            coins: this.coins,
            lives: this.lives,
            items: itemLevels,
        }
    }

}