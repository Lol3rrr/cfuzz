import Projects from './routes/Projects.svelte';
import NotFound from './routes/NotFound.svelte';
import Results from "./routes/Results.svelte";
import Running from "./routes/Running.svelte";
import Project from "./routes/Project.svelte";

export default {
    '/': Projects,
    '/running': Running,
    '/results': Results,
    '/project/:name': Project,
    // The catch-all route must always be last
    '*': NotFound
};
