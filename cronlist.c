/*
    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.

*/

#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>
#include <unistd.h>
#include <sys/types.h>
#include <pwd.h>
#include <getopt.h>
#include <stdarg.h>
#include <time.h>

#define PROGRAM_NAME "cronlist"

const char IGNORE_DOM = 'm';
const char IGNORE_DOW = 'w';
const char IGNORE_NOTHING = ' ';

struct time_entry {
  char m[60];
  char h[24];
  char dom[31];
  char mon[12];
  char dow[7];
  char ignore;
};

struct entry {
  struct time_entry te;
  char *username;
  char *command;
  struct entry *next;
};

struct time_entry EMPTY_TIME_ENTRY;

char *slurp (FILE *f)
{
  const int increment = 8192;
  int capa = increment;
  char *buf = malloc(capa);
  size_t res, offs = 0;

  for (;;) {
    res = fread(buf+offs, 1, increment, f);
    offs += res;
    if (res < increment) {
      buf[offs] = '\0';
      return buf;
    }
    capa += increment;
    buf = realloc(buf, capa);
  }
}

char *empty_string (void)
{
  char *s = malloc(1);
  s[0] = '\0';
  return s;
}

char *slurp_command (char *command)
{
  char *result;
  FILE *p = popen(command, "r");
  if (!p) return empty_string();

  result = slurp(p);
  pclose(p);
  return result;
}

char *slurp_file (char *filename)
{
  char *result;
  FILE *f = fopen(filename, "r");
  if (!f) return empty_string();

  result = slurp(f);
  fclose(f);
  return result;
}

char *next_line (char *p)
{
  while (*p && *p != '\n') p++;
  if (*p == '\n') p++;
  return p;
}

char *skip_spaces (char *p)
{
  while (*p && isspace(*p)) p++;
  return p;
}

char *skip_blanks (char *p)
{
  while (*p && isblank(*p)) p++;
  return p;
}

char *skip_irrelevant (char *p)
{
  p = skip_spaces(p);
  while (*p && *p == '#') {
    p = next_line(p);
    p = skip_spaces(p);
  }
  return p;
}

int eoln (char *p)
{
  return !*p || *p == '\n';
}

typedef int (*fn)(char *, char **);

char *read_number (char *buf, fn get_number, int *res)
{
  char *p = buf;
  if (isdigit(*p)) {
    *res = strtol(buf, &p, 10);
    if (buf == p) return NULL;
    return p;
  }
  else if (!get_number) return NULL;
  else {
    *res = get_number(buf, &p);
    if (buf == p) return NULL;
    return p;
  }
}

int find_string (char *s, char *table[])
{
  int i;
  for (i = 0; table[i]; i++) {
    if (!strncasecmp(table[i], s, strlen(table[i]))) return i;
  }
  return -1;
}

int get_from_tables (char *buf, char **tables[], char **end)
{
  int i, idx;
  for (i = 0; tables[i]; i++) {
    idx = find_string(buf, tables[i]);
    if (idx >= 0) {
      *end = buf + strlen(tables[i][idx]);
      return idx;
    }
  }
  /* not found */
  *end = buf;
  return -1;
}

static char *mon_fullnames[] = {
  "January", "February", "March", "April", "May", "June", "July", "August",
  "September", "October", "November", "December", NULL };
static char *mon_abbrevs[] = {
  "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug",
  "Sep", "Oct", "Nov", "Dec", NULL };
static char **months[] = { mon_fullnames, mon_abbrevs, NULL };


int get_month (char *buf, char **end)
{
  return get_from_tables(buf, months, end) + 1;
}

static char *dow_fullnames[] = {
  "Sunday", "Monday", "Tuesday", "Wednesday",
  "Thursday", "Friday", "Saturday", NULL };
static char *dow_abbrevs[] = {
  "Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", NULL };
static char **dows[] = { dow_fullnames, dow_abbrevs, NULL };


int get_dow_1 (char *buf, char **end)
{
  return get_from_tables(buf, dows, end);
}

int get_dow_2 (char *buf, char **end)
{
  int res = get_dow_1(buf, end);
  return res == 0 ? 7 : res;
}
  

char *read_range (char *buf, int min, int max, int offs,
                  fn get_number_1, fn get_number_2,
                  char *dest)
{
  char *p = skip_blanks(buf);
  int num1, num2, step, i;

  for (;;) {
    if (*p == '*') {
      p++;
      num1 = min; num2 = max;
    }
    else {
      p = read_number(p, get_number_1, &num1);  if (!p) return NULL;
      if (*p == '-') {
        p++;
        p = read_number(p, get_number_2, &num2); if (!p) return NULL;
      }
      else num2 = num1;
    }
    if (*p == '/') {
      p++;
      p = read_number(p, NULL, &step);
      if (!p) return NULL;
    }
    else step = 1;
    if (num1 < min || num1 > max || num2 < min || num2 > max) return NULL;

    for (i = num1; i <= num2; i += step) {
      dest[i-offs] = 1;
    }

    if (*p != ',') return p;
    else p++;
  }
}

int all_full (char *vec, int len)
{
  int i;
  for (i = 0; i < len; i++) {
    if (!vec[i]) return 0;
  }
  return 1;
}

void print_vec (char *vec, int len)
{
  int i;
  for (i = 0; i < len; i++) {
    if (vec[i]) printf(" %2d", i);
    else printf(" --");
  }
  printf("\n");
}

char *read_time_entry (char *buf, struct time_entry *res)
{
  char raw_dow[8] = { 0, 0, 0, 0, 0, 0, 0, 0 };
  char *p = skip_irrelevant(buf);
  char *keyword;
  int len;
  int fulldom, fulldow;

  if (eoln(p)) return NULL;
  *res = EMPTY_TIME_ENTRY;
  if (*p == '@') {
    p++;
    keyword = p;
    while (*p && isalpha(*p)) p++;
    len = p - keyword;
    if      (!strncmp(keyword, "reboot",   len)) /* ignore, keep empty time_entry */ ;
    else if (!strncmp(keyword, "yearly",   len)) read_time_entry("0 0 1 1 *", res);
    else if (!strncmp(keyword, "annually", len)) read_time_entry("0 0 1 1 *", res);
    else if (!strncmp(keyword, "monthly",  len)) read_time_entry("0 0 1 * *", res);
    else if (!strncmp(keyword, "weekly",   len)) read_time_entry("0 0 * * 0", res);
    else if (!strncmp(keyword, "daily",    len)) read_time_entry("0 0 * * *", res);
    else if (!strncmp(keyword, "midnight", len)) read_time_entry("0 0 * * *", res);
    else if (!strncmp(keyword, "hourly",   len)) read_time_entry("0 * * * *", res);
    else /* invalid */
      return NULL;
    return p;
  }
  /* minutes */
  p = read_range(p, 0, 59, 0, NULL, NULL, res->m);
  if (!p) return NULL;
  /* hours */
  p = read_range(p, 0, 23, 0, NULL, NULL, res->h);
  if (!p) return NULL;
  /* dom */
  p = read_range(p, 1, 31, 1, NULL, NULL, res->dom);
  if (!p) return NULL;
  /* month */
  p = read_range(p, 1, 12, 1, get_month, get_month, res->mon);
  if (!p) return NULL;
  /* dow */
  p = read_range(p, 0, 7, 0, get_dow_1, get_dow_2, raw_dow);
  if (!p) return NULL;

  if (raw_dow[7]) raw_dow[0] = 1;
  memcpy(res->dow, raw_dow, 7);

  fulldom = all_full(res->dom, 31);
  fulldow = all_full(res->dow, 7);

  if      (fulldom && !fulldow) res->ignore = IGNORE_DOM;
  else if (fulldow && !fulldom) res->ignore = IGNORE_DOW;
  else res->ignore = IGNORE_NOTHING;

  return p;
}

struct entry *add_entries (char *buf, char *username, struct entry *link)
{
  struct entry *list = link;
  char *p = buf, *q;

  while (*p) {
    struct entry *entry = malloc(sizeof(struct entry));
    q = read_time_entry(p, &entry->te);
    if (!q) {
      free(entry);
    }
    else {
      entry->next = list;
      list = entry;
      p = skip_blanks(q);
      /* username */
      if (username) entry->username = strdup(username);
      else {
        q = p;
        while (isalnum(*q)) q++;
        entry->username = strndup(p, q-p);
        p = skip_blanks(q);
      }
      /* command */
      q = p;
      while (!eoln(q)) q++;
      entry->command = strndup(p, q-p);
      p = q;
    }
    p = next_line(p);
  }
  return list;
}

void free_entry_list (struct entry *list)
{
  struct entry *p;
  while (list) {
    p = list;
    list = list->next;
    free(p->username);
    free(p->command);
    free(p);
  }
}

char *get_username (void)
{
  uid_t uid = getuid();
  struct passwd *pwd = getpwuid(uid);
  char *res;
  if (pwd) return strdup(pwd->pw_name);
  res = malloc(20);
  sprintf(res, "%u", uid);
  return res;
}

void print_te_part (char *arr, int len, int offs)
{
  int first = 1, i;
  for (i = 0; i < len; i++) {
    if (arr[i]) {
      if (!first) putchar(',');
      printf("%d", i+offs);
      first = 0;
    }
  }
}

void print_entry (struct entry *e)
{
  print_te_part(e->te.m, 60, 0);   putchar(' ');
  print_te_part(e->te.h, 24, 0);   putchar(' ');
  if (e->te.ignore == IGNORE_DOM)  putchar('-');
  print_te_part(e->te.dom, 31, 1); putchar(' ');
  print_te_part(e->te.mon, 12, 1); putchar(' ');
  if (e->te.ignore == IGNORE_DOW)  putchar('-');
  print_te_part(e->te.dow, 7, 0);  putchar(' ');
  printf(" %s %s\n", e->username, e->command);
}

struct entry *read_crontabs (int user, int system)
{
  struct entry *list = NULL;

  if (user) {
    char *buf = slurp_command("crontab -l");
    if (buf) {
      char *username = get_username();
      list = add_entries(buf, username, list);
      free(username);
      free(buf);
    }
  }

  if (system) {
    char *buf = slurp_file("/etc/crontab");
    if (buf) {
      list = add_entries(buf, NULL, list);
      free(buf);
    }
  }
  return list;
}

int match_time (struct time_entry *te, struct tm *tm) {
  return
    te->m  [ tm->tm_min  ] &&
    te->h  [ tm->tm_hour ] &&
    te->mon[ tm->tm_mon  ] &&
    ((te->ignore != IGNORE_DOM && te->dom[ tm->tm_mday-1 ]) ||
     (te->ignore != IGNORE_DOW && te->dow[ tm->tm_wday   ]));
}

int tm_ge (struct tm *tm1, struct tm *tm2)
{
  return
    tm1->tm_year > tm2->tm_year ||
    (tm1->tm_year == tm2->tm_year &&
     (tm1->tm_mon > tm2->tm_mon ||
      (tm1->tm_mon == tm2->tm_mon &&
       (tm1->tm_mday > tm2->tm_mday ||
        (tm1->tm_mday == tm2->tm_mday &&
         (tm1->tm_hour > tm2->tm_hour ||
          (tm1->tm_hour == tm2->tm_hour &&
           (tm1->tm_min > tm2->tm_min ||
            (tm1->tm_min == tm2->tm_min)))))))));
}
      
void die (char *s, ...) {
  va_list ap;

  fputs(PROGRAM_NAME ": ", stderr);
  va_start(ap, s);
  vfprintf(stderr, s, ap);
  va_end(ap);
  fputc('\n', stderr);
  exit(1);
}

void usage (void) {
  puts(PROGRAM_NAME " lists upcoming cron actions from /etc/crontab\n\
  and your personal crontab.\n\
Options:\n\
   -f  --from=DATETIME  list actions starting on or after DATETIME (default now)\n\
   -t  --to=DATETIME    list actions starting on or before DATETIME\n\
   -n  --entries=NUMBER stop after NUMBER actions (default 10)\n\
   -s  --system         show /etc/crontab only\n\
   -c  --crontab        show your personal crontab only\n\
   -h  --help           shows this help\n\
\n\
  DATETIME should be a date expression that can be passed to date(1).\n");
  exit(0);
}

void get_tm_from_date (char *datespec, struct tm *dest) {
  char *cmd = malloc(strlen(datespec) + 20);
  FILE *p;
  char number[20];
  long n;
  struct tm *stm;

  sprintf(cmd, "date -d \"%s\" '+%%s'", datespec);
  p = popen(cmd, "r");
  if (!p) die("command ‘%s’ failed", cmd);
  fgets(number, 20, p);
  pclose(p);
  if (!isdigit(number[0])) die("command ‘%s’ didn't return a meaningful value", cmd);
  n = atoi(number);
  stm = localtime(&n);
  if (!stm) die("date ‘%s’ not supported", datespec);
  *dest = *stm;
}

int mdays[] = { 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31 };

int leap (int year) {
  return (year+1900)%400 == 0 || ((year+1900)%100 != 0 && year%4 == 0);
}


int main (int argc, char *argv[])
{
  static char *shortopts = "f:t:n:csh";
  static struct option longopts[] = {
    { "from",    required_argument, NULL, 'f' },
    { "to",      required_argument, NULL, 't' },
    { "entries", required_argument, NULL, 'n' },
    { "crontab", no_argument,       NULL, 'c' },
    { "system",  no_argument,       NULL, 's' },
    { "help",    no_argument,       NULL, 'h' }
  };

  int have_from = 0, have_to = 0, have_n = 0, only_system = 0, only_crontab = 0;
  int n, outputted;
  struct tm from, to, tm, *stm;
  time_t t;
  int opt;
  char *p;
  struct entry *entries;

  opterr = 1;
  while ((opt = getopt_long(argc, argv, shortopts, longopts, NULL)) != -1) {
    switch (opt) {
    case 'f':
      have_from = 1;
      get_tm_from_date(optarg, &from);
      break;
    case 't':
      have_to = 1;
      get_tm_from_date(optarg, &to);
      break;
    case 'n':
      have_n = 1;
      n = strtol(optarg, &p, 10);
      if (*p || n < 0) die("Invalid entry count: %s", optarg);
      break;
    case 's':
      only_system = 1;
      break;
    case 'c':
      only_crontab = 1;
      break;
    case 'h':
      usage();
      break;
    default:
      /* die("Bad argument. Use ‘--help’ for a list of options"); */
      exit(1);
    }
  }
  if (only_system && only_crontab)
    die("Can't choose both --system and --crontab");
  
  if (!have_from) {
    t = time(NULL);
    stm = localtime(&t);
    from = *stm;
  }
  if (!have_to && !have_n) {
    have_n = 1;
    n = 10;
  }

  if (n <= 0) return 0;
  
  entries = read_crontabs(!only_system, !only_crontab);
  if (!entries) return 0;

  outputted = 0;
  tm = from;
  for (;;) {
    struct entry *e = entries;
    while (e) {
      if (match_time(&e->te, &tm)) {
        printf("%04d-%02d-%02d %2d:%02d  %s  %s\n",
               tm.tm_year+1900,
               tm.tm_mon+1,
               tm.tm_mday,
               tm.tm_hour,
               tm.tm_min,
               e->username,
               e->command);
        outputted++;
        if (have_n && outputted >= n) return 0;
      }
      e = e->next;
    }

    if (have_to && tm_ge(&tm, &to)) return 0;
    
    /* after one year, if nothing was output, nothing will be */
    if (outputted == 0 &&
        (tm.tm_year > from.tm_year+1 ||
         (tm.tm_year == from.tm_year+1 && tm.tm_mon > from.tm_mon)))
      return 0;
        
    /* increment tm */
    tm.tm_min++;
    if (tm.tm_min > 59) {
      tm.tm_min = 0;
      tm.tm_hour++;
      if (tm.tm_hour > 23) {
        tm.tm_hour = 0;
        tm.tm_wday = (tm.tm_wday+1)%7;
        tm.tm_mday++;
        if (tm.tm_mday > mdays[tm.tm_mon]) {
          if (tm.tm_mon != 1 ||
              tm.tm_mday == 30 ||
              !leap(tm.tm_year)) {
            tm.tm_mday = 1;
            tm.tm_mon++;
            if (tm.tm_mon > 11) {
              tm.tm_mon = 0;
              tm.tm_year++;
            }
          }
        }
      }
    }
  }
  /* never reached */

  return 0;
}
