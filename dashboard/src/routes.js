import Home from './routes/Home.svelte';
import NotFound from './routes/NotFound.svelte';
import Results from "./routes/Results.svelte";
import Run from "./routes/Run.svelte";
import Running from "./routes/Running.svelte";
import Project from "./routes/Project.svelte";

export default {
    '/': Home,
    '/run': Run,
    '/running': Running,
    '/results': Results,
    '/project/:name': Project,
    // The catch-all route must always be last
    '*': NotFound
};
