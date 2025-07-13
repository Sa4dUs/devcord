import { Component } from "@angular/core";
import { RouterOutlet } from "@angular/router";
import { MainMenuComponent } from "./mainMenu/mainMenu.component";

@Component({
    selector: "app-root",
    imports: [RouterOutlet, MainMenuComponent],
    templateUrl: "./app.html",
    styleUrl: "./app.scss",
})
export class App {
    protected title = "frontend";
}
