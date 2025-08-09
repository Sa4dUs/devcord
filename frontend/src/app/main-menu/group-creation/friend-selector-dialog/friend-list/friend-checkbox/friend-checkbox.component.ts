import { Component, Input, Output, EventEmitter } from "@angular/core";
import { FormsModule } from "@angular/forms";
import { CommonModule } from "@angular/common";

@Component({
    selector: "friend-checkbox",
    standalone: true,
    imports: [CommonModule, FormsModule],
    styleUrls: ["./friend-checkbox.component.scss"],
    templateUrl: "./friend-checkbox.component.html",
})
export class FriendCheckboxComponent {
    @Input() name!: string;
    @Input() checked: boolean = false;
    @Input() id: number | undefined;

    @Output() changed = new EventEmitter<boolean>();

    onCheckboxChange(event: Event): void {
        const checked = (event.target as HTMLInputElement).checked;
        this.changed.emit(checked);
    }
}
