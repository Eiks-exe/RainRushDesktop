import { createRunnableDevEnvironment } from "vite";

export interface IRun {
  id: string;
  userId: string;
  createdAt: string;
  updatedAt: string;
  damageDealt: number;
  damageTaken: number;
  deathReason: string;
  deaths: number;
  result: string;
  duration: number;
  difficulty: string,
  equipment: string[]; // array of equipment names
  items: string[] | null;
  kills: number;
  seed: string; 
  stageCount: number; // optional, for future use
  survivor?: string; // optional, for future use
  score?: number; // optional, for future use
 }
