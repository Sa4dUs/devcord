import { Component } from "@angular/core";
import { BubbleContainer } from "./bubble-container/bubble-container.component";
import { UserComponent } from "./user/user.component";

@Component({
    selector: "main-menu",
    standalone: true,
    imports: [BubbleContainer, UserComponent],
    templateUrl: "./main-menu.component.html",
})
export class MainMenuComponent {}
