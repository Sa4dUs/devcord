import { Component, ViewChild } from "@angular/core";
import { BubbleContainer } from "./bubble-container/bubble-container.component";
import { UserComponent } from "./user/user.component";
import { GroupCreationComponent } from "./group-creation/group-creation.component";
import { NotificationListenerService } from "../notification-listener/notification-listener.component";
import { MessageComponent } from "./message/message.component";
import { GroupLoader, Group } from "./bubble-container/group-loader/group-loader.component";

@Component({
  selector: "main-menu",
  standalone: true,
  imports: [
    BubbleContainer,
    UserComponent,
    GroupCreationComponent,
    MessageComponent,
    GroupLoader,
  ],
  templateUrl: "./main-menu.component.html",
})
export class MainMenuComponent {
  @ViewChild(BubbleContainer) bubbleContainer!: BubbleContainer;
  @ViewChild(MessageComponent) chatComponent!: MessageComponent;

  constructor(private notificationService: NotificationListenerService) {}

  addGroup(groupId: string): void {
    this.bubbleContainer.addBubble(groupId);
  }

  // Recibe los grupos del GroupLoader
  onGroupsLoaded(groups: Group[]): void {
    if (groups.length > 0) {
      const firstChannelId = groups[0].channelId;

      // Llamamos initChat con el channelId correcto
      setTimeout(() => this.chatComponent.initChat(firstChannelId));
    }
  }
}
