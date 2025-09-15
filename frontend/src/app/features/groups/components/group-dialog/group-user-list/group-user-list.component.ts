import { Component, Input, QueryList, ViewChildren } from "@angular/core";
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
    @Input() uniqueAnswer: boolean = false;

    selections = new Set<string>();
    loading = false;
    @ViewChildren(GroupUserCheckboxComponent)
    //A reference to the checkboxes, at the time it's only used for reset
    checkboxes!: QueryList<GroupUserCheckboxComponent>;

    onUserCheckboxChanged(checked: boolean, userId: string): void {
        if (!checked) {
            this.selections.delete(userId);
            return;
        }
        if (this.uniqueAnswer) {
            this.clearSelections(userId);
        }
        this.selections.add(userId);
    }
    clearSelections(userId: string): void {
        this.selections.clear();
        this.checkboxes.forEach((checkbox) => {
            if (checkbox.name !== userId) {
                checkbox.checked = false;
            }
        });
    }
}
