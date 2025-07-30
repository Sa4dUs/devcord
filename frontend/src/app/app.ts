import { Component } from "@angular/core";
import { MainMenuComponent } from "./mainMenu/mainMenu.component";

@Component({
    selector: "app-root",
    imports: [MainMenuComponent],
    templateUrl: "./app.html",
    styleUrl: "./app.scss",
})
export class App {
    protected title = "frontend";
}
