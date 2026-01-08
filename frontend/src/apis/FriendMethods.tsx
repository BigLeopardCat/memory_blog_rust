import http from "./axios.tsx";
import {Friend} from "../interface/FriendType";

export function getAllFriendsList(){
    return http({
        url: '/api/friends',
        method: 'GET'
    })
}

export function delFriends(keysToDelete: (string | number)[]){
    return  http({
        url: '/api/protected/friends',
        method: 'DELETE',
        data: keysToDelete
    })
}

export function agreeFriends(key: number, data: any){
    return http({
        url: `/api/protected/friends/${key}`,
        method: 'POST',
        data: data
    })
}

export function refuseFriends(key:number){
    return http({
        url: '/api/protected/friends',
        method: 'DELETE',
        data: [key]
    })
}

export function applyFor(value:Friend){
    return  http({
        url: '/api/public/friends',
        method: 'POST',
        data: value
    })
}

export const getFriendsList = getAllFriendsList;
