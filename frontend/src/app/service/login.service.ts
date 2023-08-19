import { Injectable } from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";
import {CookieService} from "ngx-cookie-service";
import {Router} from "@angular/router";

@Injectable({
  providedIn: 'root'
})
export class LoginService {

  constructor(
    private http: HttpClient,
    private cookies: CookieService,
    private router: Router
  ) { }

  public login(email: string, password: string) {
    this.http.post("/api/account/login", {
      email: email,
      password: password
    }).subscribe({
      next: value => {
        this.cookies.set("AUTH", "")
        this.router.navigate(["/dashboard"])
      }
    })
  }

  public logout() {
    return this.http.delete("/api/account/logout").subscribe({
      next: value => {
        this.cookies.delete("AUTH")
        this.router.navigate(["/"])
      }
    })
  }
}
