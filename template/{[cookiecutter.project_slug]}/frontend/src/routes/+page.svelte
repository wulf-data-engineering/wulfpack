<script lang="ts">
    import {dev} from '$app/environment';
    import * as auth from '$lib/auth';
    import {currentUser} from '$lib/auth';
    import {page} from "$app/state";
    import {goto} from "$app/navigation";
    import {Button} from "$lib/components/ui/button";
    import {Label} from "$lib/components/ui/label";
    import {Input} from "$lib/components/ui/input";
    import * as Card from "$lib/components/ui/card";
    import {toast} from "svelte-sonner";


    function handleError(err: unknown) {
        complete('An error occurred', err instanceof Error ? err.message : String(err), true);
    }

    function complete(text: string, description: string, error = false) {
        if (error)
            toast.error(text, {description, richColors: true, duration: 5000});
        else
            toast.success(text, {description, duration: 3000});
        loading = false;
    }

    function indicateLoading() {
        loading = true;
    }

    async function signUp() {
        indicateLoading();
        try {
            const result = await auth.signUp(email, password);
            if (result.isSignUpComplete) complete('Signed Up' ,'Successfully signed up and signed in.');
            else complete('Signed Up', 'Successfully signed up. Confirmation required.');
            console.log('Signed up:', result);
        } catch (err) {
            console.error('Error signing up:', err);
            handleError(err);
        }
    }

    async function signIn() {
        indicateLoading();
        try {
            const result = await auth.signIn(email, password);
            if (result.isSignedIn) {
                // Redirect to the originally requested page
                const redirectTo = page.url.searchParams.get('redirectTo');
                if (redirectTo)
                    await goto(redirectTo);
                else
                    complete('Signed In', 'Successfully signed in.');
            } else
                complete('Next Step', 'Please complete the sign-up process.', true);
            console.log('Signed in:', result);
        } catch (err) {
            console.error('Error signing in:', err);
            handleError(err);
        }
    }

    async function signOut() {
        indicateLoading();
        try {
            await auth.signOut();
            complete('Singed Out', 'Successfully signed out.');
        } catch (err) {
            console.error('Error signing out:', err);
            handleError(err);
        } finally {
            loading = false;
        }
    }

    let mode: 'signIn' | 'signUp' = $state('signIn');
    let loading = $state(false);
    let email = $state(dev ? 'test@wulf.technology' : '');
    let password = $state(dev ? 'Password123!' : '');

    function handleSubmit(e: Event) {
        e.preventDefault();
        if (mode === 'signIn') signIn();
        else signUp();
    }

    function toggle(newMode: 'signIn' | 'signUp') {
        return (e: Event) => {
            e.preventDefault();
            mode = newMode;
        };
    }
</script>

<svelte:head>
    <title>Home</title>
    <meta name="description" content="A Tool-Set frontend application."/>
</svelte:head>

<div class="m-auto max-w-4xl px-8 p-5">
    <div>
        <h1 class="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
            Welcome to the tool set frontend
        </h1>
        <p class="text-muted-foreground text-xl leading-7 [&:not(:first-child)]:mt-6">
            This is a sample landing page.
        </p>
    </div>

    <Card.Root class="w-full max-w-sm m-auto mt-5">

        {#if $currentUser}

            <Card.Header>
                <Card.Title>Signed In</Card.Title>
                <Card.Description>You are signed in</Card.Description>
                <Card.Action>
                    <Button variant="link" href="/account">Your account</Button>
                </Card.Action>
            </Card.Header>

            <Card.Content>
                <p>
                    You are signed in as {$currentUser.signInDetails?.loginId}. <br/>
                    <small class="text-muted-foreground">{$currentUser.userId}</small>
                </p>
            </Card.Content>

            <Card.Footer class="flex-col gap-2">
                <Button onclick={signOut} disabled={loading} variant="outline" class="w-full">Sign Out</Button>
                {#if loading}
                    Please wait…
                {/if}
            </Card.Footer>

        {:else if $currentUser === null}

            <Card.Header>
                <Card.Title>{mode === 'signIn' ? 'Sign In' : 'Sign Up'}</Card.Title>
                <Card.Description>{mode === 'signIn' ? 'Sign in to your account' : 'Create a new account'}</Card.Description>
            </Card.Header>

            <Card.Content onsubmit={handleSubmit}>
                <form>
                    <div class="flex flex-col gap-6">
                        <div class="grid gap-2">
                            <Label for="email">Email</Label>
                            <Input id="email" type="email" bind:value={email} required/>
                        </div>
                        <div class="grid gap-2">
                            <div class="flex items-center">
                                <Label for="password">Password</Label>
                            </div>
                            <Input id="password" type="password" bind:value={password} required/>
                        </div>
                    </div>
                </form>
            </Card.Content>

            <Card.Footer class="flex-col gap-2">
                <Button type="submit" disabled={loading} class="w-full" onclick={handleSubmit}>
                    {#if loading}
                        Please wait…
                    {:else}
                        {mode === 'signIn' ? 'Sign In' : 'Sign Up'}
                    {/if}
                </Button>

                <p>
                    {#if mode === 'signIn'}
                        Don’t have an account?
                        <Button variant="link" onclick={toggle('signUp')}>Sign up</Button>
                    {:else}
                        Already have an account?
                        <Button variant="link" onclick={toggle('signIn')}>Sign in</Button>
                    {/if}
                </p>

            </Card.Footer>
        {:else}
            <p>Loading…</p>
        {/if}
    </Card.Root>
</div>
