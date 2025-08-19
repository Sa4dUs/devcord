import { Component, EventEmitter, Input, Output } from "@angular/core";
import { FormsModule } from "@angular/forms";

@Component({
    selector: "group-user-checkbox",
    imports: [FormsModule],
    standalone: true,
    templateUrl: "./group-user-checkbox.component.html",
    styleUrl: "./group-user-checkbox.component.scss",
})
export class GroupUserCheckboxComponent {
    @Input() name!: string;
    @Input() checked: boolean = false;
    @Input() id: number | undefined;

    @Output() changed = new EventEmitter<boolean>();

    onCheckboxChange(event: Event): void {
        const checked = (event.target as HTMLInputElement).checked;
        this.changed.emit(checked);
    }
}
