export default class ViewModel {
    shopIsOpen: boolean = false;
    private loginScreenIsOpen: boolean = false;


    constructor() {
    }

    loginScreenOpen(loggedIn: boolean): boolean {
        return this.loginScreenIsOpen || !loggedIn;
    }

    closeLoginScreen() {
        this.loginScreenIsOpen = false;
    }

    openLoginScreen() {
        this.loginScreenIsOpen = true;
    }
}