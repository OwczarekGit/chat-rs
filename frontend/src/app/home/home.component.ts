import { Component } from '@angular/core';
import {HttpClient} from "@angular/common/http";

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.css']
})
export class HomeComponent {
  constructor(private http: HttpClient) {
  }

  public login() {
    this.http.post("/api/account/login", {
      email: "a@a.com",
      password: "1234",
    }).subscribe()
  }

  public logout() {
    this.http.delete("/api/account/logout").subscribe()
  }
}
