<script lang="ts">
    import { path, navigate } from './lib/router';

    import Home from './routes/Home.svelte';
    import About from './routes/About.svelte';
    import Projects from './routes/Projects.svelte';
    import Blog from './routes/Blog.svelte';
    import NotFound from './routes/NotFound.svelte';

    import type { Component } from 'svelte';

    const routes: Record<string, Component> = {
        '/': Home,
        '/about': About,
        '/projects': Projects,
        '/blog': Blog,
        '404': NotFound,
    };

    $: Page = routes[$path] ?? NotFound;
</script>

<main>
    <nav>
        <a href="/" on:click={(e) => navigate(e, '/')}>Home</a>
        <a href="/about" on:click={(e) => navigate(e, '/about')}>About</a>
        <a href="/projects" on:click={(e) => navigate(e, '/projects')}>Projects</a>
        <a href="/blog" on:click={(e) => navigate(e, '/blog')}>Blog</a>
    </nav>

    <svelte:component this={Page} />
</main>
