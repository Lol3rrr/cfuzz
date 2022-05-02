import { Writable, writable } from "svelte/store";
import type { Project } from "./api";
import * as api from "./api";

export const projects: Writable<Array<Project>> = writable([]);

export async function updateProjects() {
    let n_projects = await api.load_projects();
    projects.set(n_projects);
}