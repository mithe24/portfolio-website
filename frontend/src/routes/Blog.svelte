<script lang="ts">
    import { marked } from "marked";
    import { onMount } from "svelte";

    type Post = { id: string; title: string; content: string };

    let posts: Post[] = [];
    let page = 1;
    let loading = false;
    let has_more = true;
    let sentinel: HTMLElement;

    async function load_more() {
        if (loading || !has_more) return;
        loading = true;

        /* TODO */
        // const res = await fetch('http//mithe.dk/posts?page=${page}');
        const res = await fetch('http://localhost:8080/posts?page=1${page}');
        const data = await res.json();

        posts = [...posts, ...data.posts];
        has_more = data.has_more;
        page++;
        loading = false;
    }

    onMount(() => {
        load_more();

        const observer = new IntersectionObserver((entries) => {
            if (entries[0].isIntersecting) load_more();
        }, { rootMargin: '200px' });

        observer.observe(sentinel);
        return () => observer.disconnect();
    });
</script>

<h1>Blog</h1>
{#each posts as post (post.id)}
    <article>
        <h2>{post.title}</h2>
        {@html marked.parse(post.content)}
    </article>
{/each}

{#if loading}
    <p>Loading...</p>
{/if}

<div bind:this={sentinel}></div>
