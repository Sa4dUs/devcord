import { Component, ViewChild } from "@angular/core";
import { GroupCreationComponent } from "../components/group-creation/group-creation.component";
import { NotificationService } from "../../../core/services/notification-service";
import { BubbleContainer } from "../components/bubble-container/bubble-container.component";
import { UserComponent } from "../components/user/user.component";

@Component({
    selector: "main-menu",
    standalone: true,
    imports: [BubbleContainer, UserComponent, GroupCreationComponent],
    templateUrl: "./main-menu.page.html",
})
export class MainMenuPage {
    //TODO:@AlexGarciaPrada As it was suggested by Sa4dus, supercomponent who use a notificationListenerService
    constructor(private notificationService: NotificationService) {}
    @ViewChild(BubbleContainer)
    bubbleContainer!: BubbleContainer;

    addGroup(groupId: string): void {
        this.bubbleContainer.addBubble(groupId);
    }
}
