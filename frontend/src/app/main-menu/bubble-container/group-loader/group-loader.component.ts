import { CommonModule, isPlatformBrowser } from "@angular/common";
import { HttpClient, HttpHeaders, HttpParams } from "@angular/common/http";
import {
  ChangeDetectorRef,
  Component,
  Inject,
  Output,
  PLATFORM_ID,
  EventEmitter,
} from "@angular/core";
import { ErrorsHandling } from "../../../errors/errors";
import { SERVER_ROUTE } from "../../../../environment/environment.secret";

export interface Group {
  groupId: string;
  channelId: string;
}

@Component({
  selector: "group-loader",
  standalone: true,
  imports: [CommonModule],
  templateUrl: "./group-loader.component.html",
  styleUrls: ["./group-loader.component.scss"],
})
export class GroupLoader {
  @Output() groupsLoaded = new EventEmitter<Group[]>();
  @Output() channelSelected = new EventEmitter<string>();

  loading = false;
  error: string | null = null;
  groups: Group[] = [];

  constructor(
    private http: HttpClient,
    @Inject(PLATFORM_ID) private platformId: object,
    private errorsMap: ErrorsHandling,
    private cdRef: ChangeDetectorRef
  ) {}

  ngOnInit(): void {
    this.loadGroups();
  }

  loadGroups(): void {
    if (!isPlatformBrowser(this.platformId)) {
      this.loading = false;
      return;
    }

    this.loading = true;
    this.error = null;

    const token = localStorage.getItem("token");
    if (!token) {
      this.error = "You're not logged in";
      this.loading = false;
      return;
    }

    const params = new HttpParams().set("from", "0").set("to", "20");

    this.http
      .get<{ id: string; channel_id: string }[]>(
        `${SERVER_ROUTE}/api/group/user-groups`,
        {
          headers: new HttpHeaders().set("Authorization", `Bearer ${token}`),
          params,
        }
      )
      .subscribe({
        next: (data) => {
          this.groups = data.map((g) => ({
            groupId: g.id,
            channelId: g.channel_id,
          }));

          this.loading = false;
          this.groupsLoaded.emit(this.groups);

          // Emitimos el channelId del primer grupo (o cualquiera que quieras)
          if (this.groups.length > 0) {
            this.channelSelected.emit(this.groups[0].channelId);
          }

          this.cdRef.detectChanges();
        },
        error: (err) => {
          this.error = `Error loading groups: ${err.message || err.status}`;
          this.loading = false;
        },
      });
  }
}
