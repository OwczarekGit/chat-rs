import { Injectable } from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";
import {ProfileSearchResult} from "../data/profile-search-result";

@Injectable({
  providedIn: 'root'
})
export class SearchService {

  constructor(private http: HttpClient) { }

  public searchUsers(phrase: string): Observable<ProfileSearchResult[]>{
    return this.http.get<ProfileSearchResult[]>(`/api/search/profile/${phrase}`)
  }
}
