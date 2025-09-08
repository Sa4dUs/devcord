import { Injectable } from "@angular/core";
import { HttpClient, HttpHeaders, HttpParams } from "@angular/common/http";
import { SERVER_ROUTE } from "../../../environment/environment.secret";
import { Observable } from "rxjs";

export interface MessageFormat {
  header: string;
  info: Record<string, string>;
}

@Injectable({ providedIn: 'root' })
export class MessageHistoryService {
  constructor(private http: HttpClient) {}

  getMessages(channelId: string, from = 0, to = 50): Observable<any[]> {
    const token = localStorage.getItem("token");
    if (!token) throw new Error("No hay token, login requerido");

    const params = new HttpParams().set("from", from).set("to", to);
    return this.http.get<any[]>(
      `${SERVER_ROUTE}/api/${channelId}`,
      {
        headers: new HttpHeaders().set("Authorization", `Bearer ${token}`),
        params
      }
    );
  }
}
