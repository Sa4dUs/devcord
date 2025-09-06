import { Component } from "@angular/core";
import { CommonModule } from "@angular/common";
import { ActivatedRoute } from "@angular/router";
import { MessageListenerService } from "../message-listener/message-listener.service"; 

@Component({
  selector: "app-message",
  standalone: true,
  imports: [CommonModule],
  templateUrl: "./message.component.html",
  styleUrls: ["./message.component.scss"],
})
export class MessageComponent {
  groupId = "";

  constructor(
    private route: ActivatedRoute,
    private messageListener: MessageListenerService // <--- inyectamos
  ) {}

  ngOnInit() {
    const groupId = "abc123"; // o el id real del grupo
    this.messageListener.init(groupId); // ahora funciona
  }
}
