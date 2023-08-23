import { Injectable } from '@angular/core';
import {Observable} from "rxjs";
import {HttpClient} from "@angular/common/http";
import {UserProfile} from "../data/user-profile";

@Injectable({
  providedIn: 'root'
})
export class ProfileService {

  public currentUser?: UserProfile

  constructor(private http: HttpClient) { }

  public getMyProfile(handler?: (profile: UserProfile) => void) {
    this.http.get<UserProfile>("/api/profile/me").subscribe({
      next: value =>
      {
        this.currentUser = value
        if (handler != null)
          handler(value)
      }
    })
  }

  public changeUsername(username: string) {
    this.http.put("/api/profile", {
      username: username
    }).subscribe()
  }
}
