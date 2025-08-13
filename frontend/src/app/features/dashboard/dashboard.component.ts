import { Component } from "@angular/core";
import { UserComponent } from "../user/user.component";
import { BubbleContainer } from "../../shared/components/bubble/bubble-container/bubble-container.component";

@Component({
    selector: "dashboard",
    standalone: true,
    imports: [BubbleContainer, UserComponent],
    templateUrl: "./dashboard.component.html",
})
export class DashboardComponent {}
