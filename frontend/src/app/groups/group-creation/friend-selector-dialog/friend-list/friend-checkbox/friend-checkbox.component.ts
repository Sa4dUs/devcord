import { Component, Input } from "@angular/core";
import { FormsModule } from "@angular/forms";
import { CommonModule } from "@angular/common";

@Component({
    selector: "friend-checkbox",
    standalone: true,
    imports: [CommonModule, FormsModule],
    styleUrl: "./friend-checkbox.component.scss",
    templateUrl: "./friend-checkbox.component.html",
})
export class FriendCheckboxComponent {
    @Input() name!: string;
    @Input() checked: boolean = false;
}
