import { Octokit } from "@octokit/rest";

interface AdvancedPatchOptions {
  octokit: Octokit;
  owner: string;
  repo: string;
  branch: string;
  path: string;
  targetSearch: string;
  replacementContent: string;
  commitMessage: string;
}

/**
 * Motor Determinístico Avançado (Autossuficiente e Resiliente)
 */
export async function applyAdvancedDeterministicPatch(options: AdvancedPatchOptions): Promise<boolean> {
  const { octokit, owner, repo, branch, path, targetSearch, replacementContent, commitMessage } = options;

  try {
    // 1. Busca o conteúdo atual do arquivo no GitHub
    const { data: fileData } = await octokit.repos.getContent({
      owner,
      repo,
      path,
      ref: branch,
    });

    if (!('content' in fileData)) {
      throw new Error(`O caminho ${path} não é um arquivo válido.`);
    }

    const currentContent = Buffer.from(fileData.content, 'base64').toString('utf8');

    // 2. Verifica se o trecho alvo existe
    if (!currentContent.includes(targetSearch)) {
      console.log(`[Deterministic Engine] Padrão alvo não encontrado em ${path}. Nenhuma alteração aplicada.`);
      return false;
    }

    const updatedContent = currentContent.replace(targetSearch, replacementContent);

    if (updatedContent === currentContent) {
      return false;
    }

    // 3. Grava e commita a alteração de forma determinística via API do GitHub
    await octokit.repos.createOrUpdateFileContents({
      owner,
      repo,
      path,
      message: commitMessage,
      content: Buffer.from(updatedContent).toString('base64'),
      sha: fileData.sha,
      branch,
    });

    console.log(`[Deterministic Engine] Sucesso! Patch determinístico aplicado em ${path}.`);
    return true;

  } catch (error: any) {
    console.error(`❌ [Deterministic Engine Error] Falha ao patchar ${path}:`, error.message);
    return false;
  }
}

