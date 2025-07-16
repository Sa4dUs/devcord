import { Component, OnInit, Inject, PLATFORM_ID } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';
import { HttpClient, HttpParams } from '@angular/common/http';
import { CommonModule } from '@angular/common';

interface FriendList {
  username: string;
  created_at: string;
}


@Component({
  selector: 'app-friendship-friend',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './friendship-friend.component.html',
  styleUrls: ['./friendship-friend.component.scss']
})
export class FriendshipFriendComponent implements OnInit {
  requests: FriendList[] = [];
  loading = true;
  error: string | null = null;

  constructor(
    private http: HttpClient,
    @Inject(PLATFORM_ID) private platformId: Object
  ) {}

ngOnInit() {

  if (!isPlatformBrowser(this.platformId)) {
    this.loading = false;
    return;
  }


  const token = localStorage.getItem('token');

  if (!token) {
    this.error = 'No estás logueado. Inicia sesión para ver solicitudes enviadas.';
    console.error(' No hay token');
    this.loading = false;
    return;
  }

  const params = new HttpParams().set('from', '0').set('to', '20'); //ya sé que es una chapuza, pero así se queda...

  this.http.get<FriendList[]>('http://lamoara.duckdns.org:6969/api/user/friendship/friends', {
    headers: { Authorization: `Bearer ${token}` },
    params
  }).subscribe({
    next: data => {
        console.log(data);
    this.requests = data.map(d => ({
    username: d.username,
    created_at: d.created_at
    }));


      this.loading = false;
    },
    error: err => {
      console.error(' Error al hacer la petición:', err);
      this.error = `Error al cargar solicitudes: ${err.message || err.status}`;
      this.loading = false;
    }
  });
}

get showList(): boolean {
  return !this.loading && !this.error && this.requests.length > 0;
}

get showEmpty(): boolean {
  return !this.loading && !this.error && this.requests && this.requests.length === 0;
}

}
