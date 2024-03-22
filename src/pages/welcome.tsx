import { Button } from '~/components/ui/button';

export function WelcomePage() {
    const testClipboard = async () => {};

    return (
        <div>
            <Button onClick={testClipboard}>test clipboard</Button>
        </div>
    );
}
