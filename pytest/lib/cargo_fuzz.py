# Running `cargo-fuzz` based fuzzers.
import os
import sys
import subprocess


def run(dir: str, fuzz_target: str) -> int:
    args = ('cargo', 'fuzz', 'run', fuzz_target, '--', '-len_control=0'
            '-prefer_small=0', '-max_len=4000000', '-rss_limit_mb=10240')
    os.environ['RUSTC_BOOTSTRAP'] = '1'
    try:
        # libfuzzer has a -max_total_time flag however it does not measure time
        # compilation takes.  Because of that, rather than using that option
        # we’re handling timeout over the entire command ourselves.
        return subprocess.call(args,
                               cwd=os.path.join('..', dir),
                               timeout=_get_timeout())
    except subprocess.TimeoutExpired:
        print('No failures found.')
        return 0


def _get_timeout():
    timeout = os.environ.get('NAYDUCK_TIMEOUT')
    if timeout:
        try:
            n = int(timeout)
            if n > 60:
                # Reserve five seconds for the time this script takes; it’s way
                # more than enough.
                return n - 5
        except ValueError:
            pass
        print(f'Invalid NAYDUCK_TIMEOUT value ‘{timeout}’, ignoring.',
              file=sys.stderr)
    print(
        'No valid NAYDUCK_TIMEOUT environment variable found.\n'
        'Test will run until failure is found or it’s interrupted.',
        file=sys.stderr)
    return None
