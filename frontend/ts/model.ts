import { GameRessource } from "../bindings/GameRessource";
import { ItemLevelRessource } from "../bindings/ItemLevelRessource";
import { Enemy, Field, GameModel, GameState, Item, Projectile, Ship } from "../pkg";
import Api from "./api";
import HotkeyHandler, { Hotkey } from "./hotkeys";
import ViewModel from "./view_model";

export * from "./hotkeys";
export * from "../pkg";

export default class Model {
    private gameModel: GameModel;
    private apiModel: Api;
    private viewModel: ViewModel;
    private hotkeyModel: HotkeyHandler;
    private lastFrameGameOver: boolean = false;
    private gameOverCallbacks: (() => void)[] = [];


    constructor() {
        this.gameModel = GameModel.new();
        this.apiModel = new Api();
        this.viewModel = new ViewModel();
        this.hotkeyModel = new HotkeyHandler();

        this.apiModel.registerOnLogin(() => {
            this.loadLatestGame();
        });
        this.apiModel.registerOnLogin(() => {
            console.log("Logged in as " + this.username);
        });
        this.registerOnGameOver(() => {
            const id = this.gameModel.get_id();
            if (id)
                this.apiModel.setGameEnded(id);
        })
    }

    registerHotkeys(hotkeys: Hotkey[]) {
        this.hotkeyModel.registerHotkeys(hotkeys);
    }

    registerOnGameOver(callback: () => void) {
        this.gameOverCallbacks.push(callback);
    }

    handleHotkey(e: KeyboardEvent) {
        if (this.isLoginScreenOpen) return;
        this.hotkeyModel.handleHotkey(e);
    }

    tick(): void {
        this.gameModel.tick();
    }

    async startGame() {
        this.gameModel.start_game();
        let gameId = await this.apiModel.createGame(this.gameRessource);
        if (gameId) {
            this.gameModel.set_id(gameId);
        }
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
        this.saveGame();
    }

    resumeGame(): void {
        this.gameModel.resume_game();
    }

    buyItem(itemId: number): void {
        this.gameModel.buy_item(itemId);
        this.saveGame();
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

    register(username: string, password: string): void {
        this.apiModel.register(username, password);
    }

    login(username: string, password: string): void {
        this.apiModel.login(username, password);
    }

    logout(): void {
        this.apiModel.logout();
    }

    deleteAccount(): void {
        this.apiModel.deleteAccount();
    }

    closeLoginScreen(): void {
        this.viewModel.closeLoginScreen();
    }

    openLoginScreen(): void {
        this.viewModel.openLoginScreen();
    }

    private loadGameFromRessource(gameRessource: GameRessource) {
        if (!gameRessource.ended)
            this.gameModel.load_game(gameRessource);
    }

    saveGame(): void {
        if (this.loggedIn)
            this.apiModel.saveGame(this.gameRessource);
    }

    private async loadLatestGame() {
        let gameRessource = await this.apiModel.loadGame();

        if (gameRessource)
            this.loadGameFromRessource(gameRessource)
    }

    async endGame() {
        this.gameModel.end_game();
        if (this.loggedIn) {
            await this.apiModel.setGameEnded(this.gameModel.get_id()!);
        }
    }

    // Getters

    get hotkeys(): Hotkey[] {
        return this.hotkeyModel.hotkeys;
    }

    get loading(): boolean {
        return this.apiModel.loading;
    }

    get validatingToken(): boolean {
        return this.apiModel.validatingToken;
    }

    get isLoginScreenOpen(): boolean {
        return this.viewModel.loginScreenOpen(this.loggedIn);
    }

    get loggedIn(): boolean {
        return this.apiModel.loggedIn;
    }

    get username(): string {
        return this.apiModel.username;
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
        return this.apiModel.errorMessage || this.gameModel.get_error_message();
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