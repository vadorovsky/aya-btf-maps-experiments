#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>

#define TASK_COMM_LEN 16

struct {
	__uint(type, BPF_MAP_TYPE_HASH);
	__type(key, unsigned int);
	__type(value, unsigned int);
	__uint(max_entries, 1024);
} pid_map SEC(".maps");

struct fork_ctx {
	char parent_comm[TASK_COMM_LEN];
	unsigned int parent_pid;
	char child_comm[TASK_COMM_LEN];
	unsigned int child_pid;
};

SEC("tp/sched/sched_process_fork")
int fork(void *ctx) {
	struct fork_ctx *fork_ctx = (struct fork_ctx *)ctx;
	unsigned int parent_pid = fork_ctx->parent_pid;
	unsigned int child_pid = fork_ctx->child_pid;
	
	bpf_printk("fork! parent pid: %u, child pid: %u", parent_pid, child_pid);
	
	bpf_map_update_elem(&pid_map, &parent_pid, &child_pid, BPF_NOEXIST);
	
	return 0;
}

char _license[] SEC("license") = "GPL";
