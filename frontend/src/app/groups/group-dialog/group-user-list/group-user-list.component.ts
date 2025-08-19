import { Component, Input } from "@angular/core";
import { GroupUserCheckboxComponent } from "./group-user-checkbox/group-user-checkbox.component";
import { CommonModule } from "@angular/common";

@Component({
    selector: "group-user-list",
    imports: [CommonModule, GroupUserCheckboxComponent],
    standalone: true,
    templateUrl: "./group-user-list.component.html",
    styleUrl: "./group-user-list.component.scss",
})
export class GroupUserListComponent {
    @Input() users: string[] = [];
    selections = new Set<string>();
    loading = false;

    onUserCheckboxChanged(checked: boolean, userId: string): void {
        if (checked) {
            this.selections.add(userId);
            console.log("Seleccionado:", userId, this.selections);
        } else {
            this.selections.delete(userId);
            console.log("Deseleccionado:", userId, this.selections);
        }
    }
}
