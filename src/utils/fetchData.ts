import axios, { AxiosResponse } from "axios"; 
import { IUser } from "../interfaces/IUser";
import { invoke } from "@tauri-apps/api/core";
import { IRun } from "../interfaces/Irun";

interface loginResponse {
  message: string;
  user: IUser;
  token: string;    
}

axios.defaults.baseURL = "http://localhost:8080"; // Replace with your API base URL
export const checkApi = async (url: string) => {
  try {
    const { status } = await axios.get(url);
    status === 200 ? Promise.resolve("Api is up") : Promise.reject("Api is down");
  } catch (error) {
    console.error("Error checking API:", error);
    return Promise.reject("Api is down");
  }
}

export const login = async (identifier: string, password: string) => {
  try {
    console.log("Logging in with identifier:", identifier);
    const body = {
      identifier: identifier,
      password: password,
    };

    const response : AxiosResponse = await axios.post("/api/auth/login", body);
    if(!response) {
      return;
    }
    const { data } = response;
    console.log("Login successful:", data);
    return { id : data.user.id, username: data.user.username, email: data.user.email, token: data.token };
  } 
  catch (error) {
    console.error("Error logging in:", error);
    return; 
  }
}

export const login_back = async (identifier: string, password: string) => {
  try {
    const response = await invoke<loginResponse>("login", { identifier: identifier, password: password });
    if (!response) {
      return;
    }
    console.log("Login successful:", response);
    const { user, token } = response;
    return { id: user.id, username: user.username, email: user.email, token: token };
  } 
  catch (error) {
    console.error("Error logging in:", error);
    return; 
  }
}

export const fetchUserRuns = async (userId: string) => {
  try {
    const response = await axios.get(`/api/users/${userId}/runs`);
    if (!response) {
      return;
    }
    const { data } = response;
    console.log("Fetched user runs:", data);
    return data
  } 
  catch (error) {
    console.error("Error fetching user runs:", error);
    return [];
  }
}
