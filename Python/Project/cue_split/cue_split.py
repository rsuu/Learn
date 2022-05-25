# Changes over the original:
#
# + Support both Python 2 & 3.
# * fixed location of bitrate parameters.
# * Fixed track duration so it does't cuts tracks short, nor starts them early
#   (Margin of error for .flac source files should at most be around +/- 0.013 secs).
# + Added pre_gap support.

# ls
# xxx.cue xxx.flac
# python3 cue_split.py xxx.cue | parallel



import sys


n = 1

cue_file = sys.argv[n]

d = open(cue_file).read().splitlines()

general = {}

tracks = []

current_file = None


def index2seconds(line):
    mm, ss, ff = list(map(int, ' '.join(line.strip().split(' ')[2:]).replace('"', '').split(':')))
    return 60 * mm + ss + ff / 75.0 # each FF is 1/75 of a sec


def replace_chars(string, chars=r':\/|<>?*"', replacement=''):
    """
    Return `string` with any char in the `chars` replaced by `replacement`.

    Defaults to replace problematic/invalid chars for filenames/paths.
    """
    for c in string:
        if c in chars:
            string = string.replace(c, replacement)
    return string


for line in d:
    if line.startswith('REM GENRE '):
        general['genre'] = ' '.join(line.split(' ')[2:])
    if line.startswith('REM DATE '):
        general['date'] = ' '.join(line.split(' ')[2:])
    if line.startswith('PERFORMER '):
        general['artist'] = ' '.join(line.split(' ')[1:]).replace('"', '')
    if line.startswith('TITLE '):
        general['album'] = ' '.join(line.split(' ')[1:]).replace('"', '')
    if line.startswith('FILE '):
        current_file = ' '.join(line.split(' ')[1:-1]).replace('"', '')

    if line.startswith('  TRACK '):
        track = general.copy()
        track['track'] = int(line.strip().split(' ')[1], 10)

        tracks.append(track)

    if line.startswith('    TITLE '):
        tracks[-1]['title'] = ' '.join(line.strip().split(' ')[1:]).replace('"', '')
    if line.startswith('    PERFORMER '):
        tracks[-1]['artist'] = ' '.join(line.strip().split(' ')[1:]).replace('"', '')
    if line.startswith('    INDEX 00 '):
        tracks[-1]['pre_gap'] = index2seconds(line)
    if line.startswith('    INDEX 01 '):
        tracks[-1]['start'] = index2seconds(line)

for i in range(len(tracks) - 1):
    if 'pre_gap' in tracks[i + 1]:
        tracks[i]['duration'] = tracks[i + 1]['pre_gap'] - tracks[i]['start']
    else:
        tracks[i]['duration'] = tracks[i + 1]['start'] - tracks[i]['start']

for track in tracks:
    metadata = {
        'artist': track['artist'],
        'title': track['title'],
        'album': track['album'],
        'track': str(track['track']) + '/' + str(len(tracks))
    }

    if 'genre' in track:
        metadata['genre'] = track['genre']
    if 'date' in track:
        metadata['date'] = track['date']

    cmd = 'ffmpeg'
    cmd += ' -i "%s" -c copy' % current_file
    cmd += ' -ss %.2d:%.2d:%09.6f' % (track['start'] / 60 / 60, track['start'] / 60 % 60, track['start'] % 60)

    if 'duration' in track:
        cmd += ' -t %.2d:%.2d:%09.6f' % (track['duration'] / 60 / 60, track['duration'] / 60 % 60, track['duration'] % 60)

    cmd += ' ' + ' '.join('-metadata %s="%s"' % (k, v) for (k, v) in metadata.items())
    cmd += replace_chars(' '"'\%.2d - %s - %s.opus'"'' % (track['track'], track['artist'], track['title']))

    print(cmd)
