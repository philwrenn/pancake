class App {

    constructor() {
        this.browsers = [];

        this.setupBrowserSelect();
        this.setupRememberSelect();
        this.setupOpenButton();
        this.setupExitButton();
        this.setupWindowListeners();

        this.applyColorPreference();
        this.invoke({ cmd: "Init" });
    }

    addBrowserOption(browser) {
        let browserOpt = new Option();
        browserOpt.value = browser.key;
        browserOpt.innerText = browser.display;
        this.browserSelect.appendChild(browserOpt);
    }

    applyColorPreference() {
        this.preferDarkStyle(
            window.matchMedia &&
            window.matchMedia("(prefers-color-scheme: dark)").matches
        );
    }

    init(config) {
        this.config = config;
        this.setTheme();
        this.setBrowsers();
    }

    invoke(cmd) {
        window.external.invoke(JSON.stringify(cmd));
    }

    launchBrowser() {
        let key = this.browserSelect.options[
            this.browserSelect.selectedIndex
            ].value;
        let remember = this.rememberSelect.options[
            this.rememberSelect.selectedIndex
            ].value;
        this.invoke({
            cmd: "LaunchBrowser",
            exec: this.browsers[key].exec,
            envs: this.browsers[key].envs,
            key: key,
            remember: remember
        });
        this.invoke({ cmd: "Exit" });
    }

    preferDarkStyle(preferDark) {
        const bodyElem = document.getElementById("pancake");
        if (preferDark) {
            bodyElem.classList.add("dark");
            bodyElem.classList.remove("light");
        }
        else {
            bodyElem.classList.add("light");
            bodyElem.classList.remove("dark");
        }
    }

    setBrowsers() {
        this.config.browsers.forEach((browser) => {
            this.browsers[browser.key] = browser;
            this.addBrowserOption(browser);
        });
        if (this.config.browsers.length > 0) {
            this.browserSelect.selectedIndex = this.config.browsers[0].value;
        }
    }

    setTheme() {
        if (this.config.theme == "light") {
            this.preferDarkStyle(false)
        }
        else if (this.config.theme == "dark") {
            this.preferDarkStyle(true)
        }
    }

    setupBrowserSelect() {
        this.browserSelect = document.getElementById("browserSelect");
        this.browserSelect.addEventListener("keypress", (event) => {
            if (event.keyCode === 13) {
                this.launchBrowser();
            }
        });
        this.browserSelect.focus();
    }

    setupRememberSelect() {
        this.rememberSelect = document.getElementById("rememberSelect");
    }

    setupExitButton() {
        let openButton = document.getElementById("exitButton");
        openButton.addEventListener("click", (event) => {
            this.invoke({ cmd: "Exit" })
        });
    }

    setupOpenButton() {
        let openButton = document.getElementById("openBrowser");
        openButton.addEventListener("click", (event) => {
            this.launchBrowser();
        });
    }

    setupWindowListeners() {
        window.addEventListener("keypress", (event) => {
            if (event.keyCode === 27) {
                this.invoke({ cmd: "Exit" });
            }
        });
    }
}

window.app = new App();