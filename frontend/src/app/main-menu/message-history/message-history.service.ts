
import { tap } from 'rxjs/operators';

import { Injectable } from "@angular/core";
import { HttpClient, HttpHeaders, HttpParams } from "@angular/common/http";
import { SERVER_ROUTE } from "../../../environment/environment.secret";
import { Observable } from "rxjs";
import { MessageFormat } from "../message-listener/message-listener.service";


@Injectable({ providedIn: 'root' })
export class MessageHistoryService {
  constructor(private http: HttpClient) {}

getMessages(channelId: string, from = 0, to = 50): Observable<MessageFormat[]> {
  const token = localStorage.getItem("token");
  if (!token) throw new Error("No hay token, login requerido");

  const params = new HttpParams()
    .set("from", String(from))
    .set("to", String(to));

  return this.http.get<MessageFormat[]>(
    `${SERVER_ROUTE}/api/message/${channelId}`, // asegúrate de la ruta correcta
    {
      headers: new HttpHeaders().set("Authorization", `Bearer ${token}`),
      params
    }
  ).pipe(
    tap(response => console.log("Historial recibido del server:", response))
  );
}

}
