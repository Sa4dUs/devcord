import { Component, ViewChild } from "@angular/core";
import { BubbleContainer } from "./bubble-container/bubble-container.component";
import { UserComponent } from "./user/user.component";
import { GroupCreationComponent } from "./group-creation/group-creation.component";
@Component({
    selector: "main-menu",
    standalone: true,
    imports: [BubbleContainer, UserComponent, GroupCreationComponent],
    templateUrl: "./main-menu.component.html",
})
export class MainMenuComponent {
    @ViewChild(BubbleContainer)
    bubbleContainer!: BubbleContainer;

    addGroup(groupId: string): void {
        this.bubbleContainer.addBubble(groupId);
    }
}
