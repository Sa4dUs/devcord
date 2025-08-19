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
    checkboxes!: QueryList<GroupUserCheckboxComponent>;

    onUserCheckboxChanged(checked: boolean, userId: string): void {
        if (checked) {
            if (this.uniqueAnswer) {
                this.clearSelections(userId);
            }
            this.selections.add(userId);
            console.log("Seleccionado:", userId, this.selections);
        } else {
            this.selections.delete(userId);
            console.log("Deseleccionado:", userId, this.selections);
        }
    }
    clearSelections(userId: string): void {
        console.log(userId);
        this.selections.clear();
        this.checkboxes.forEach((checkbox) => {
            if (checkbox.name !== userId) {
                checkbox.checked = false;
            }
        });
        console.log("Todos los checkboxes desmarcados");
    }
}
