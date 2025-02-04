const assert = require('assert');
const taskcluster = require('taskcluster-client');
const gql = require('graphql-tag');
const testing = require('taskcluster-lib-testing');
const helper = require('../helper');
const createTaskQuery = require('../fixtures/createTask.graphql');
const loader = require('../../src/loaders/tasks');

helper.secrets.mockSuite(testing.suiteName(), [], function(mock, skipping) {
  helper.withDb(mock, skipping);
  helper.withClients(mock, skipping);
  helper.withServer(mock, skipping);
  helper.resetTables(mock, skipping);

  suite('tasks loaders', function() {
    // Make sure we still get tasks even if we end up loading some tasks that don't exist
    test('load multiple tasks while gracefully handling errors', async function() {
      const client = helper.getHttpClient();
      const taskId = taskcluster.slugid();

      // 1. create task
      await client.mutate({
        mutation: gql`${createTaskQuery}`,
        variables: {
          taskId,
          task: helper.makeTaskDefinition(),
        },
      });

      const taskLoader = loader({ queue: helper.clients().queue, index: helper.clients.index }).task;

      // 2. get tasks
      const [firstTask, taskThatDoesNotExist] = await Promise.allSettled([
        taskLoader.load(taskId),
        taskLoader.load('taskId-that-does-not-exist'),
      ]);

      assert.equal(firstTask.status, 'fulfilled');
      assert.equal(firstTask.value.taskId, taskId);
      assert.equal(taskThatDoesNotExist.status, 'rejected');
      assert(taskThatDoesNotExist.reason instanceof Error);
    });
  });
});
