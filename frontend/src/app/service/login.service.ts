import { Injectable } from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";
import {CookieService} from "ngx-cookie-service";
import {Router} from "@angular/router";
import {ProfileService} from "./profile.service";
import {UserProfile} from "../data/user-profile";

@Injectable({
  providedIn: 'root'
})
export class LoginService {


  constructor(
    private http: HttpClient,
    private cookies: CookieService,
    private router: Router,
    private profileService: ProfileService,
  ) {
    this.profileService.getMyProfile()
  }

  public login(email: string, password: string) {
    this.http.post("/api/account/login", {
      email: email,
      password: password
    }).subscribe({
      next: value => {

        this.profileService.getMyProfile()

        this.cookies.set("AUTH", "")
        this.router.navigate(["/dashboard"])
      }
    })
  }

  public logout() {
    this.cookies.delete("AUTH")
    return this.http.delete("/api/account/logout").subscribe({
      next: value => {
        // this.router.navigate(["/"])
      }
    })
  }
}
